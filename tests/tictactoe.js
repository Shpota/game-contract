const anchor = require("@project-serum/anchor");
const assert = require('chai').assert
const expect = require('chai').expect

describe("TicTacToe", () => {
    let provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.TicTacToe;
    const {SystemProgram} = anchor.web3;

    const startGame = async () => {
        const game = anchor.web3.Keypair.generate();
        let tx = await program.methods.startGame().accounts({
            game: game.publicKey,
            user: provider.wallet.publicKey,
            systemProgram: SystemProgram.programId,
        }).signers([game]).rpc();
        assert.ok(tx);
        return game
    }

    const joinGame = async (game) => {
        const player = anchor.web3.Keypair.generate();
        let joinGameTx = await program.methods.joinGame().accounts({
            game: game.publicKey,
            user: player.publicKey,
        }).signers([player]).rpc();
        assert.ok(joinGameTx);
        return player;
    }

    const firstPlayerSetValue = async (game, index) => await program.methods.setValue(index)
            .accounts({ game: game.publicKey, user: provider.wallet.publicKey }).rpc()

    const secondPlayerSetValue = async (game, secondPlayer, index) => await program.methods.setValue(index)
            .accounts({ game: game.publicKey, user: secondPlayer.publicKey }).signers([secondPlayer]).rpc()

    it("Start game", async () => {
        const game = await startGame()

        const account = await program.account.game.fetch(game.publicKey);
        expect(account.state).to.eql([-1, -1, -1, -1, -1, -1, -1, -1, -1]);
        expect(account.firstPlayer.toBase58()).to.equal(provider.wallet.publicKey.toBase58());
        expect(account.secondPlayer.toBase58()).to.equal(anchor.web3.PublicKey.default.toBase58());
        expect(account.xPlayer.toBase58()).to.equal(anchor.web3.PublicKey.default.toBase58());
        expect(account.winner.toBase58()).to.equal(anchor.web3.PublicKey.default.toBase58());
    });

    it("Join game", async () => {
        const game = await startGame()

        const player = await joinGame(game)

        let account = await program.account.game.fetch(game.publicKey);
        expect(account.firstPlayer.toBase58()).to.equal(provider.wallet.publicKey.toBase58());
        expect(account.secondPlayer.toBase58()).to.equal(player.publicKey.toBase58())
    });

    it("Should fail to join when second player is the same as first", async () => {
        const game = await startGame()
        const firstPlayer = provider.wallet.publicKey;

        try {
            await program.methods.joinGame().accounts({ game: game.publicKey, user: firstPlayer }).rpc()
            assert.fail("Joining with the same player public key is supposed to fail");
        } catch (e) {
            assert(e.message.includes("Second Player must be different from the first player"))
        }
    });

    it("Should fail to join when there is already a second player joined", async () => {
        const game = await startGame()
        await joinGame(game)

        try {
            await joinGame(game)
            assert.fail("Joining a game when there is already the second player joined is supposed to fail");
        } catch (e) {
            assert(e.message.includes("Cannot join when there is already a second player"))
        }
    });

    it("Should fail to set value when second player not joined the game", async () => {
        const game = await startGame()

        try {
            await firstPlayerSetValue(game, 2)
            assert.fail("The setValue call is supposed to fail because the second player hasn't joined");
        } catch (e) {
            assert(e.message.includes("Second player has not joined the game"))
        }
    });

    it("Should set value", async () => {
        const game = await startGame()
        await joinGame(game);

        await firstPlayerSetValue(game, 2);
        let account = await program.account.game.fetch(game.publicKey);
        expect(account.state[2]).to.eql(1);
    });

    it("should be able to set value for both users", async () => {
        const game = await startGame()
        let secondPlayer = await joinGame(game)

        await firstPlayerSetValue(game, 2);
        await secondPlayerSetValue(game, secondPlayer, 3)
        let account = await program.account.game.fetch(game.publicKey);
        expect(account.state[2]).to.eql(1);
        expect(account.state[3]).to.eql(0);
    });

    it("should not be able to override a value", async () => {
        const game = await startGame()
        let secondPlayer = await joinGame(game)

        await firstPlayerSetValue(game, 2);
        await secondPlayerSetValue(game, secondPlayer, 3)

        try {
            await firstPlayerSetValue(game, 3);
            assert.fail("The setValue call is supposed to fail because the value is set");
        } catch (e) {
            assert(e.message.includes("Cannot Override a previously set value"))
        }
    });

    it("should not be able to make two moves", async () => {
        const game = await startGame()
        await joinGame(game)
        await firstPlayerSetValue(game, 2);

        try {
            await firstPlayerSetValue(game, 3);
            assert.fail("The setValue call is supposed to fail because the value is set");
        } catch (e) {
            assert(e.message.includes("Another player is supposed to set value"))
        }
    });

    it("should get a winner", async () => {
        const game = await startGame()
        let secondPlayer = await joinGame(game);
        await firstPlayerSetValue(game, 0);
        await secondPlayerSetValue(game, secondPlayer, 3);
        await firstPlayerSetValue(game, 1);
        await secondPlayerSetValue(game, secondPlayer, 4);
        await firstPlayerSetValue(game, 2);

        let account = await program.account.game.fetch(game.publicKey);
        expect(account.winner.toBase58()).to.equal(provider.wallet.publicKey.toBase58());
    });
});