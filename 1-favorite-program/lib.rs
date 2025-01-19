// allows us to import everything from 'Anchor' rather than typing the name.
// pulls all the prelude into our current scope.
use anchor_lang::prelude::*;

// '::' is just rust separator for namespaces, similar to '.' in other languages.

// a program also has a program id also called an Address. We need to set up a program address for our smart contract.
// this is automatically during deployment when use Solana Playground
// declare_id!("CRGr5Y2bChPmkVShA9E3DrLTTQ1MUvS9TDf6fFNADgjC"); -- initial code
declare_id!("CRGr5Y2bChPmkVShA9E3DrLTTQ1MUvS9TDf6fFNADgjC"); // automatically added by solana playground - our program's deployed address.

/* 
The ANCHOR_DISCRIMINATOR_SIZE is something that is written to every account on the blockchain by an Anchor Program. 
It specifies the type of account it is. 
It's used by Anchor for some of it's checks. And when we save things to the blockchain, 
we'll need 8 bytes plus the size of whatever we're saving.
 */
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8; // 8 - size in bytes every anchor account needs minimum

// The great thing about Anchor is that we can take a regular Rust program and turn it into a Anchor program using a Macro.

#[program] // Solana Program Macro. And inside this module each function would become an Anchor instruction handler.
           // Each Instuction handler would have a context which is all the account that the people calling the instruction have provided plus other arguments.
pub mod favorites {
    use super::*; // using all the imported ones

    pub fn set_favorites(
        context: Context<SetFavorites>, // the first argument to our Instruction Handler is the Context<SetFavorites> which are the acccounts.
        number: u64,                    // followed by the options they would like to save.
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        // Reasults a return of nothing
        // This function is our actual instruction handler, the thing that people are gonna call from the instruction in their Solana Transactions.
        msg!("Greetings from {}", context.program_id); // Anchor's built-in message macro. Similar to console.log()
        let user_public_key: Pubkey = context.accounts.user.key();

        msg!(
            "User {}'s favorite number is {}, favorite color is {}, and their hobbies are {:?}",
            user_public_key,
            number,
            color,
            hobbies
        );

        // write the information into favorite account provided.

        context.accounts.favorites.set_inner(Favorites {
            // set_inner - would write the information into the account.
            number,
            color,
            hobbies,
        });

        // return with ok

        Ok(()) // no need for the ';' and Rust would return the actual ok response. They write info to the blockchain rather than returning it.
    }
}

// struct for writing what we want to write to the blockchain.
#[account] // using this macro since we're saving this to an account.
#[derive(InitSpace)] // to let Anchor know how big Favorites is, this gives all our instances of Favorites the InitSpace attribute
                     // which is the total space used by all the items inside.
                     // We'll also need to specify the size of each individual items inside to get that as strings could be of any size,
                     // thus we need to specify a MAX_LENGTH

pub struct Favorites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5, 50)] //vector of size 5 and each of 50 bytes.
    pub hobbies: Vec<String>,
}

/* when people call our set_favorites function, they need to provide a list of accounts that they need to change on the blockchain. 
One of the things that makes Solana Blockchain great is that if there's a bunch of people over at A who are running a transaction involing their accounts
and there's a bunch of people at B running a different transaction involing their accounts, the transactions need not block eachother, there's no overlap evolved.
Solana can process them at the same time without waiting for the other one to be finished.
 */

// struct to store the information,
// this struct of accounts is for our set Favorites instruction handler, the tradition is to name the struct the same as out Instruction Handler in TitleCase.
// structs in Rust have TitleCase.

#[derive(Accounts)] // so that Anchor knows this is our account struct and that specifies the accounts people need to provide along with the instruction
pub struct SetFavorites<'info> {
    #[account(mut)] // we're specifying some options for this account.
    // mut - mutable, because the person that signs the instruction to SetFavorites is going to pay to create their favorites account on the block chain.
    pub user: Signer<'info>, // info - indicates that items will live for the lifetime of a Solana account info object.
    //This is just Rust having rules about how long things should be kept in memory.


    // also we'll need the person running the SetFavorites to specify the Favorites account they want to write to.
    // Doesn't mean that we will let them write to the specified account but rather we need them to specify an account.
    #[account(
        init_if_needed, // init_if_needed - make the account if it doesn't already exist.
        payer = user, // payer - who pays to create the account, 'user' - person who Signed the transaction.
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE, // space - how much space the account needs, 
                                                                  //when declaring the struct we used the derive(InitSpace) which would help us to calc the size for the Favorites account.
        seeds = [b"favorites", user.key().as_ref()], // seeds - we will need to have seeds which are used to give this account an address on the blockchain, this is a PDA. Unlike a regular user account this isn't a public key.
                                                     // The address for this is actually made based on some seeds that we provide.
                                                     // Here, we're using the text favorites as bytes and user's own key. This means that if I'm storing my favorites: I'll store that under the address made from 'favorites' as bytes,
                                                     // and user's own public key, 
        
        bump // bump - used to calculate those seeds.
    )]
    pub favorites: Account<'info, Favorites>, // an account of the Favorites struct we made earlier.

    // last account we'll need people to specify is just the system program. Used for so many things, It's not the system program, it's the token program.
    // The program will last the lifetime of the infor and it is a program of type system.
    pub system_program: Program<'info, System>,
}

/*
 the program ensures that the person is already signing the program has to be writing to their own favorites account. 
 Logic - in the 'seed' we're use the user's key where user is the one who signed the transaction. 
 Good example of the things that Anchor provides like smart safe defaults.

 Controls over what accounts people are able to write to are handled by the programmer.
 */
