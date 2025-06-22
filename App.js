import React from "react";
import { HttpAgent, Actor } from "@dfinity/agent";
import { idlFactory as backend_idl, canisterId as backend_id } from "../../../declarations/icp_dapp_backend";

const agent = new HttpAgent({ host: "http://localhost:4943" });

const backend = Actor.createActor(backend_idl, {
  agent,
  canisterId: backend_id,
});

function App() {
  const handleCreate = async () => {
    await backend.create_item(1, "Bhawana’s first task 👋");
    alert("Item created!");
  };

  return (
    <div>
      <h1>ICP DApp</h1>
      <button onClick={handleCreate}>Create Item</button>
    </div>
  );
}

export default App;
