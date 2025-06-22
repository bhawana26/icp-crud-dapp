import React from "react";
import { HttpAgent, Actor } from "@dfinity/agent";
import { idlFactory as backend_idl, canisterId as backend_id } from "../../declarations/icp_dapp_backend";

const agent = new HttpAgent({ host: "http://localhost:4943" });


// 👇 Only for local development (important!)
if (process.env.DFX_NETWORK === "local") {
  agent.fetchRootKey().catch(err => {
    console.warn("Unable to fetch root key. Check if local replica is running.");
    console.error(err);
  });
}

const icp_dapp_backend = Actor.createActor(backend_idl, {
  agent,
  canisterId: backend_id,
});

function App() {
  const handleCreate = async () => {
    await icp_dapp_backend.create_item(1, "Bhawana's first task");
    alert("Item created!");
  };

  const handleRead = async () => {
    const result = await icp_dapp_backend.read_item(1);
    alert("Fetched item: " + result);
  };

  const handleUpdate = async () => {
    await  icp_dapp_backend.update_item(1, "Updated task");
    alert("Item updated!");
  };

  const handleDelete = async () => {
    await  icp_dapp_backend.delete_item(1);
    alert("Item deleted!");
  };

return (
<div style = {{ textAlign: "center" , marginTop: "50px" }}>
     <h1>ICP DApp CRUD</h1>
     <button onClick={handleCreate}>Create Item</button>
     <button onClick={handleRead}>Read Item</button>
     <button onClick={handleUpdate}>Update Item</button>
     <button onClick={handleDelete}>Delete Item</button>
</div>
);
}
export default App;
