import { useState } from "react";
import { Wallet } from "./components/Wallet/Wallet";
import { ModalProps, ModalState } from "./types";

function App() {
  const [modal, setModal] = useState<ModalProps>({ state: ModalState.None });

  return (
    <>
      <Wallet modal={modal} setModal={setModal} />
    </>
  );
}

export default App;
