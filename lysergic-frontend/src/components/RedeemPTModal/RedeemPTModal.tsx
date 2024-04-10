import { Connection, PublicKey, TransactionInstruction } from "@solana/web3.js";
import { Numberu64, makeRedeemFromPTInstruction, signTransactionInstruction } from "../../hooks/api";
import { ModalProps, ModalState } from "../../types";
import { useState } from "react";

type Props = {
  setModal: React.Dispatch<React.SetStateAction<ModalProps>>;
  connection: Connection;
  redeemer: PublicKey;
  lsuMint: PublicKey;
  lsuVault: PublicKey;
  redeemerLsuAta: PublicKey;
  redeemerPtAta: PublicKey;
  maturityDate: Date;
};

function RedeemPTModal(props: Props) {
  const { setModal, connection, redeemer, lsuMint, lsuVault, redeemerLsuAta, redeemerPtAta, maturityDate } = props;

  const [amount, setAmount] = useState<string>();

  return (
    <div className="modal">
      <div className="modal-header">
        Redeem PT Modal
        <div className="close" onClick={() => setModal({ state: ModalState.None })}>
          &#10006;
        </div>
      </div>
      <input type="number" min={0} onChange={(e) => setAmount(e.target.value)} />
      <button
        onClick={() => {
          if (Number(amount) > 0 && amount !== undefined) {
            makeRedeemFromPTInstruction(
              redeemer,
              lsuMint,
              lsuVault,
              redeemerLsuAta,
              redeemerPtAta,
              new Numberu64(amount),
              maturityDate
            ).then((res: TransactionInstruction) => signTransactionInstruction(connection, [], redeemer, [res]));
          }
        }}
      >
        REDEEM PT
      </button>
    </div>
  );
}

export default RedeemPTModal;
