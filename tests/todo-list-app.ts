import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TodoListApp } from "../target/types/todo_list_app";
import { assert } from "chai";

describe("todo-list-app", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TodoListApp as Program<TodoListApp>;
  
  const author = program.provider as anchor.AnchorProvider;
  it("Can create a task" , async () => {
    const task = anchor.web3.Keypair.generate();
    const tx = await program.methods
    .addingTask("First task")
    .accounts({
      task: task.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([task])
    .rpc();

    console.log("Your transaction signature", tx);

    const taskAccount = await program.account.task.fetch(task.publicKey);
    console.log("Task account: ", taskAccount);

    assert.equal(
      taskAccount.author.toBase58(),
      author.wallet.publicKey.toBase58(),
    );

    assert.equal(taskAccount.text, "First task");
    assert.equal(taskAccount.isDone, false);
    assert.ok(taskAccount.createdAt);
    assert.ok(taskAccount.updatedAt);
  });

  // it("cannot create a task with more than 400 characters", async () => {
  //   const task = anchor.web3.Keypair.generate();
  //   try {
  //     await program.methods
  //       .addingTask("You are awesome".repeat(100))
  //       .accounts({
  //         task: task.publicKey,
  //         author: author.wallet.publicKey,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .signers([task])
  //       .rpc();
  //     assert.fail("Expected an error");
  //   } catch (err) {
  //     assert.equal(err.toString(), "Error: failed to send transaction");
  //   }
  // });

  // it("Can update a task", async () => {
  //   const task = anchor.web3.Keypair.generate();
  //   const update_tx = await program.methods.updatingTask(true)
  //   .accounts({
  //     task: task.publicKey,
  //     author: author.wallet.publicKey,
  //   })
  //   .signers([author])
  //   .rpc();

  //   console.log("Update signature", update_tx);

  //   const taskAccount = await program.account.task.fetch(task.publicKey);
  //   console.log("Task account: ", taskAccount);

  //   assert.equal(taskAccount.isDone, true);
  //   assert.ok(taskAccount.updatedAt > taskAccount.createdAt);
  // });
});
