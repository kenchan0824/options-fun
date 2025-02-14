import * as anchor from "@coral-xyz/anchor";
import { web3, Program, BN } from "@coral-xyz/anchor";
import { CallsProgram } from "../target/types/calls_program";
import { SimpleUser, findProgramAddress, u8 } from "@solardev/simple-web3";
const assert = require("assert");


describe("calls-program", () => {

    const provider = anchor.AnchorProvider.env()
    anchor.setProvider(provider);

    const program = anchor.workspace.CallsProgram as Program<CallsProgram>;
    const owner = SimpleUser.generate(provider.connection)
    let marketPda;

    before(async () => {

        await owner.faucet();
        await owner.mint("POPCAT", 12).mint("USDC").commit();
        [marketPda,] = findProgramAddress(
            program.programId,
            [
                "market",
                owner.tokens["POPCAT"].mint,
                owner.tokens["USDC"].mint,
                u8(40),
                u8(7)
            ]
        );

    });

    it("Market Created", async () => {

        const tx = await program.methods.createMarket(
                40, // premium rate
                7, // expiry days
                new BN(1_000_000_000_000), // lot notional amount
            )
            .accounts({
                market: marketPda,
                baseMint: owner.tokens["POPCAT"].mint,
                quoteMint: owner.tokens["USDC"].mint,
                owner: owner.publicKey,
            })
            .signers([owner])
            .rpc();

        const market = await program.account.market.fetch(marketPda);

        assert.ok(market.owner.toBase58() === owner.publicKey.toBase58());
        assert.ok(market.baseMint.toBase58() === owner.tokens["POPCAT"].mint.toBase58());
        assert.ok(market.quoteMint.toBase58() === owner.tokens["USDC"].mint.toBase58());
        assert.ok(market.premiumRate === 40);
        assert.ok(market.expiryDays === 7);
        assert.ok(market.lotNotionalBaseAmount.toNumber() === 1_000_000_000_000);
        assert.ok(market.lotPremiumBaseAmount.toNumber() === 40_000_000_000);
        assert.ok(market.baseDecimals === 12);
        assert.ok(market.quoteDecimals === 9);

    });
});
