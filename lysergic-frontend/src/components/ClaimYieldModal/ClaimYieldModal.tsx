import { Connection, PublicKey, TransactionInstruction } from "@solana/web3.js";
import { Numberu64, makeClaimYieldInstruction, signTransactionInstruction } from "../../hooks/api";
import { ModalProps, ModalState } from "../../types";
import { useState } from "react";

type Props = {
  setModal: React.Dispatch<React.SetStateAction<ModalProps>>;
  connection: Connection;
  claimer: PublicKey;
  lsuMint: PublicKey;
  lsuVault: PublicKey;
  claimerLsuAta: PublicKey;
  claimerYtAta: PublicKey;
  maturityDate: Date;
};

function ClaimYieldModal(props: Props) {
  const { setModal, connection, claimer, lsuMint, lsuVault, claimerLsuAta, claimerYtAta, maturityDate } = props;

  const [amount, setAmount] = useState<string>();

  return (
    <div className="modal">
      <div className="modal-header">
        Claim Yield
        <div className="close" onClick={() => setModal({ state: ModalState.None })}>
          &#10006;
        </div>
      </div>
      <input type="number" min={0} onChange={(e) => setAmount(e.target.value)} />
      <button
        onClick={() => {
          if (Number(amount) > 0 && amount !== undefined) {
            makeClaimYieldInstruction(
              claimer,
              lsuMint,
              lsuVault,
              claimerLsuAta,
              claimerYtAta,
              new Numberu64(amount),
              maturityDate
            ).then((res: TransactionInstruction) => signTransactionInstruction(connection, [], claimer, [res]));
          }
        }}
      >
        CLAIM YIELD
      </button>
    </div>
  );
}

export default ClaimYieldModal;
