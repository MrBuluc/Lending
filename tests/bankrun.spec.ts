import { describe, it } from "node:test";
import { BN, Program } from "@coral-xyz/anchor";
import { BankrunProvider } from "anchor-bankrun";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { createAccount, createMint, mintTo } from "spl-token-bankrun";
import { PythSolanaReceiver } from "@pythnetwork/pyth-solana-receiver";

import { ProgramTestContext, startAnchor, BanksClient } from "solana-bankrun";

import { Connection, PublicKey, Keypair } from "@solana/web3.js";

// @ts-ignore
import IDL from "../target/idl/lending.json";
import { Lending } from "../target/types/lending";
import { BankrunContextWrapper } from "../bankrun-utils/bankrunConnection";

describe("Lending Smart Contract Test", async () => {
    let signer: Keypair;
    let usdcBankAccount: PublicKey;
    let solBankAccount: PublicKey;

    let solTokenAccount: PublicKey;
    let context: ProgramTestContext;
    let provider: BankrunProvider;
    let program: Program<Lending>;
    let banksClient: BanksClient;
    let bankrunContextWrapper: BankrunContextWrapper;

    const pyth = new PublicKey("7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE");

    const devnetConnection = new Connection("https://api.devnet.solana.com");
    const accountInfo = await devnetConnection.getAccountInfo(pyth);

    context = await startAnchor("", [{ name: "lending", programId: new PublicKey(IDL.address) }], [{ address: pyth, info: accountInfo }]);

    provider = new BankrunProvider(context);

    const SOL_PRICE_FEED_ID = "0xeaa020c61cc479712813461ce153894a96a6c00b21ed0cfc2798d1f9a9e9c94a";

    bankrunContextWrapper = new BankrunContextWrapper(context);

    const connection = bankrunContextWrapper.connection.toConnection();

    const pythSolanaReceiver = new PythSolanaReceiver({ connection, wallet: provider.wallet });

    const solUsdPriceFeedAccount = pythSolanaReceiver.getPriceFeedAccountAddress(0, SOL_PRICE_FEED_ID).toBase58();

    const solUsdPriceFeedAccountPubkey = new PublicKey(solUsdPriceFeedAccount);

    const feedAccountInfo = await devnetConnection.getAccountInfo(solUsdPriceFeedAccountPubkey);

    context.setAccount(solUsdPriceFeedAccountPubkey, feedAccountInfo);

    program = new Program<Lending>(IDL as Lending, provider);

    banksClient = context.banksClient;

    signer = provider.wallet.payer;

    const mintUSDC = await createMint(
        // @ts-ignore
        banksClient,
        signer,
        signer.publicKey,
        null,
        2);

    const mintSol = await createMint(
        // @ts-ignore
        banksClient,
        signer,
        signer.publicKey,
        null,
        2
    );

    //await bankrunContextWrapper.fundKeypair(signer, 10 ** 9);

    [usdcBankAccount] = PublicKey.findProgramAddressSync([Buffer.from("treasury"), mintUSDC.toBuffer()], program.programId);

    [solBankAccount] = PublicKey.findProgramAddressSync([Buffer.from("treasury"), mintSol.toBuffer()], program.programId);

    console.log("USDC Bank Account:", usdcBankAccount.toBase58());

    console.log("SOL Bank Account:", solBankAccount.toBase58());

    it("Test Init User", async () => {
        console.log("Create User Account:", await program.methods.initUser(mintUSDC).accounts({ signer: signer.publicKey }).rpc({ commitment: "confirmed" }));
    });

    it("Test Init and Fund Bank", async () => {
        const initUSDCBankTx = await program.methods.initBank(new BN(1), new BN(1)).accounts({ signer: signer.publicKey, mint: mintUSDC, tokenProgram: TOKEN_PROGRAM_ID }).rpc({ commitment: "confirmed" });

        console.log("Create USDC Bank Account:", initUSDCBankTx);

        const amount = 10_000 * 10 ** 9;

        const mintTx = await mintTo(
            // @ts-ignore
            banksClient,
            signer,
            mintUSDC,
            usdcBankAccount,
            signer,
            amount);

        console.log("Mint USDC to Bank Success:", mintTx);
    });

    it("Test Init and Fund Sol Bank", async () => {
        console.log("Create SOL Bank Account:", await program.methods.initBank(new BN(2), new BN(1)).accounts({ signer: signer.publicKey, mint: mintSol, tokenProgram: TOKEN_PROGRAM_ID }).rpc({ commitment: "confirmed" }));

        const amount = 10_000 * 10 ** 9;

        const mintTx = await mintTo(
            // @ts-ignore
            banksClient,
            signer,
            mintSol,
            solBankAccount,
            signer,
            amount
        );

        console.log("Mint SOL to Bank Success:", mintTx);
    });

    it("Create and Fund Token Account", async () => {
        const USDCTokenAccount = await createAccount(
            // @ts-ignore
            banksClient,
            signer,
            mintUSDC,
            signer.publicKey);

        console.log("USDC Token Account:", USDCTokenAccount);

        const amount = 10_000 * 10 ** 9;

        const mintUSDCTx = await mintTo(
            // @ts-ignore
            banksClient,
            signer,
            mintUSDC,
            USDCTokenAccount,
            signer,
            amount
        );

        console.log("Mint USDC to User:", mintUSDCTx);
    });

    it("Test Deposit", async () => {
        console.log("Deposit USDC:", await program.methods.deposit(new BN(100_000_000_000)).accounts({ signer: signer.publicKey, mint: mintUSDC, tokenProgram: TOKEN_PROGRAM_ID }).rpc({ commitment: "confirmed" }));
    });

    it("Test Borrow", async () => {
        const [userAccount] = PublicKey.findProgramAddressSync([signer.publicKey.toBuffer()], program.programId);
        const [bankAccount] = PublicKey.findProgramAddressSync([mintSol.toBuffer()], program.programId);

        console.log("Borrow SOL:", await program.methods.borrow(new BN(100_000_000)).accounts({
            signer: signer.publicKey,
            mint: mintSol,
            bank: bankAccount,
            bankTokenAccount: solBankAccount,
            userAccount: userAccount,
            priceUpdate: solUsdPriceFeedAccount,
            tokenProgram: TOKEN_PROGRAM_ID,
        }).rpc({ commitment: "confirmed" }));
    });

    it("Test Repay", async () => {
        const [userAccount] = PublicKey.findProgramAddressSync([signer.publicKey.toBuffer()], program.programId);
        const [bankAccount] = PublicKey.findProgramAddressSync([mintSol.toBuffer()], program.programId);

        console.log("Repay SOL:", await program.methods.repay(new BN(100_000_000)).accounts({
            signer: signer.publicKey,
            mint: mintSol,
            bank: bankAccount,
            bankTokenAccount: solBankAccount,
            userAccount: userAccount,
            tokenProgram: TOKEN_PROGRAM_ID,
        }).rpc({ commitment: "confirmed" }));
    });

    it("Test Withdraw", async () => {
        console.log("Withdraw USDC:", await program.methods.withdraw(new BN(100)).accounts({ signer: signer.publicKey, mint: mintUSDC, tokenProgram: TOKEN_PROGRAM_ID }).rpc({ commitment: "confirmed" }));
    });
});