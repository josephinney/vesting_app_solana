import * as anchor from '@coral-xyz/anchor'
import {Program} from '@coral-xyz/anchor'
import {Keypair, PublicKey} from '@solana/web3.js'
import {Tokenvesting} from '../target/types/tokenvesting'
import { createMint, createAccount, TOKEN_PROGRAM_ID, mintTo, } from '@solana/spl-token'

describe('tokenvesting', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const payer = provider.wallet as anchor.Wallet

  const program = anchor.workspace.Tokenvesting as Program<Tokenvesting>


  // Update every time runs the test
  const authority = new PublicKey("7MtREUnR1jcwXKmZFUi7hvq7MvW2tp7RZjxvHyVtty84"); //YO
  const mint = new PublicKey("Ei23PUj8KHApAj3BZiM5kGkbWQFgRzakkFeWt7U3yH9E");
  const treasuryTokenAccount = new PublicKey("GEpYrZt9AHyjUBcwG2GLpMk95osYVXqXoLs2NS9ThmVq")
  const token_program = new PublicKey(TOKEN_PROGRAM_ID)

  it('Initialize Vesting Account', async () => {

    // Crear un Mint de token SPL
    // const mint = await createMint(
    //   provider.connection,
    //   payer.payer,
    //   payer.publicKey,
    //   null,
    //   9
    // )


    // Crear Treasure Token Account
    // const treasuryTokenAccount = await createAccount(
    //   provider.connection,
    //   payer.payer,
    //   mint,
    //   owner
    // )

    // Minting some Tokens para la treasury account
    // const TxSx = await mintTo(
    //   provider.connection,
    //   payer.payer,
    //   mint,
    //   treasuryTokenAccount,
    //   authority,
    //   1e11
    // )
  
    // await program.methods
    // .createVestingAccount("My Company")
    // .accounts(
    //   {
    //     mint: mint,
    //     tokenProgram: TOKEN_PROGRAM_ID
    //   }
    // )
    // .rpc()

    let [vestingAccountPDA] = await PublicKey.findProgramAddressSync(
      [
        Buffer.from("My Company")
      ],
      program.programId
    )

    console.log("PDA: ", vestingAccountPDA)

    const vestingAccount = await program.account.vestingAccount.fetch(vestingAccountPDA)

    console.log("The Initialized Vesting Account: ", vestingAccount)

    expect(vestingAccount.owner).toEqual(authority);
    expect(vestingAccount.mint).toEqual(mint);
    expect(vestingAccount.treasuryTokenAccount).toEqual(treasuryTokenAccount);
    expect(vestingAccount.companyName).toEqual("My Company");

  })

  it('Create Employee Account', async () => {

    // Definir los tiempos para el vesting
    const now = Math.floor(Date.now()/1000)
    const startTime = new anchor.BN(now)
    const cliffTime = new anchor.BN(now + 1200)       // 1 minuto después del inicio
    const endTime = new anchor.BN(now + 3600)      // Vesting de 1 hora
    const totalAmount = new anchor.BN(1000)       // Tokens asignados al empleado
    const beneficiary = new PublicKey("7MtREUnR1jcwXKmZFUi7hvq7MvW2tp7RZjxvHyVtty84") 

    const [vestingAccountPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("My Company")],
      program.programId
    )

    // await program.methods
    // .createEmployeeAccount(
    //   startTime,
    //   endTime,
    //   totalAmount,
    //   cliffTime
    //   )
    // .accounts({
    //   vestingAccount: vestingAccountPDA,
    //   beneficiary: beneficiary
    // })
    // .rpc()

    const [employeeAccountPDA] = await PublicKey.findProgramAddressSync(
      [
        Buffer.from("employee_vesting"),
        beneficiary.toBuffer(),
        vestingAccountPDA.toBuffer()
      ],
      program.programId
    )

    //console.log("PDA EMPLOYEE: ",employeeAccountPDA)

    const employeeAccount = await program.account.employeeAccount.fetch(employeeAccountPDA)
    
    // console.log("Employee Account Data: ")
    // console.log("beneficiary: ", employeeAccount.beneficiary.toBase58());
    // console.log("start time: ", employeeAccount.startTime.toNumber())
    // console.log("end time: ", employeeAccount.endTime.toNumber())
    // console.log("cliff time: ", employeeAccount.cliffTime.toNumber())
    // console.log("vesting account: ", employeeAccount.vestingAccount.toBase58())
    // console.log("total amount: ", employeeAccount.totalAmount.toNumber())
    // console.log("total withdrawn: ", employeeAccount.totalWithdrawn.toNumber())
    // console.log("bump: ", employeeAccount.bump)

  })

  it('Claim Tokens', async () => {

   }) 
  
  
})


//const tokenvestingKeypair = Keypair.generate()

//const currentCount = await program.account.tokenvesting.fetch(tokenvestingKeypair.publicKey)

    //expect(currentCount.count).toEqual(0)
