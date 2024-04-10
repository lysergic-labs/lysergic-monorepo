import { Connection, PublicKey, TransactionInstruction } from "@solana/web3.js";
import { Numberu64, makeTokenizeYieldInstruction, signTransactionInstruction } from "../../hooks/api";
import { ModalProps, ModalState } from "../../types";
import { useState } from "react";

type Props = {
  setModal: React.Dispatch<React.SetStateAction<ModalProps>>;
  connection: Connection;
  buyer: PublicKey;
  lsuMint: PublicKey;
  maturityDate: Date;
  lsuVault: PublicKey;
  buyerLsuAta: PublicKey;
  buyerPtAta: PublicKey;
  buyerYtAta: PublicKey;
};

function TokenizeYieldModal(props: Props) {
  const { setModal, connection, buyer, lsuMint, maturityDate, lsuVault, buyerLsuAta, buyerPtAta, buyerYtAta } = props;

  const [amount, setAmount] = useState<string>();

  return (
    <div className="modal">
      <div className="modal-header">
        Tokenize Yield
        <div className="close" onClick={() => setModal({ state: ModalState.None })}>
          &#10006;
        </div>
      </div>
      <input type="number" min={0} onChange={(e) => setAmount(e.target.value)} />
      <button
        onClick={() => {
          if (Number(amount) > 0 && amount !== undefined) {
            makeTokenizeYieldInstruction(
              buyer,
              lsuMint,
              maturityDate,
              lsuVault,
              buyerLsuAta,
              buyerPtAta,
              buyerYtAta,
              new Numberu64(amount)
            ).then((res: TransactionInstruction) => signTransactionInstruction(connection, [], buyer, [res]));
          }
        }}
      >
        TOKENIZE YIELD
      </button>
    </div>
  );
}

export default TokenizeYieldModal;
