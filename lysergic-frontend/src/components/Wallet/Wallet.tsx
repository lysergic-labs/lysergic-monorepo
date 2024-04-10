import "./Wallet.scss";

import { useMemo } from "react";

// import { createDefaultAuthorizationResultCache, SolanaMobileWalletAdapter } from "@solana-mobile/wallet-adapter-mobile";
import { WalletAdapterNetwork } from "@solana/wallet-adapter-base";
import { ConnectionProvider, WalletProvider, useWallet } from "@solana/wallet-adapter-react";
import { WalletDisconnectButton, WalletModalProvider, WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import {
  //   GlowWalletAdapter,
  PhantomWalletAdapter,
  //   SlopeWalletAdapter,
  SolflareWalletAdapter,
  TorusWalletAdapter,
} from "@solana/wallet-adapter-wallets";
import { Connection, clusterApiUrl } from "@solana/web3.js";

// Default styles that can be overridden by your app
import "@solana/wallet-adapter-react-ui/styles.css";
import Lysergic from "../Lysergic/Lysergic";
import { ModalProps, ModalState } from "../../types";
import TokenizeYieldModal from "../TokenizeYieldModal/TokenizeYieldModal";
import RedeemModal from "../RedeemYieldModal/RedeemModal";
import RedeemPTModal from "../RedeemPTModal/RedeemPTModal";
import ClaimYieldModal from "../ClaimYieldModal/ClaimYieldModal";

type Props = {
  modal: ModalProps;
  setModal: React.Dispatch<React.SetStateAction<ModalProps>>;
};

export function Wallet(props: Props) {
  const { modal, setModal } = props;

  const { wallet } = useWallet();

  const connection = new Connection("https://api.devnet.solana.com");

  const renderedModal = () => {
    switch (modal.state) {
      case ModalState.TokenizeYield:
        return (
          wallet?.adapter.publicKey &&
          modal.lsuMint &&
          modal.maturityDate &&
          modal.lsuVault &&
          modal.userLsuAta &&
          modal.userPtAta &&
          modal.userYtAta && (
            <TokenizeYieldModal
              setModal={setModal}
              connection={connection}
              buyer={wallet?.adapter.publicKey}
              lsuMint={modal.lsuMint}
              maturityDate={modal.maturityDate}
              lsuVault={modal.lsuVault}
              buyerLsuAta={modal.userLsuAta}
              buyerPtAta={modal.userPtAta}
              buyerYtAta={modal.userYtAta}
            />
          )
        );
      case ModalState.RedeemYield:
        return (
          wallet?.adapter.publicKey &&
          modal.lsuMint &&
          modal.maturityDate &&
          modal.lsuVault &&
          modal.userLsuAta &&
          modal.userPtAta &&
          modal.userYtAta && (
            <RedeemModal
              setModal={setModal}
              connection={connection}
              redeemer={wallet?.adapter.publicKey}
              lsuMint={modal.lsuMint}
              maturityDate={modal.maturityDate}
              lsuVault={modal.lsuVault}
              redeemerLsuAta={modal.userLsuAta}
              redeemerPtAta={modal.userPtAta}
              redeemerYtAta={modal.userYtAta}
            />
          )
        );
      case ModalState.RedeemFromPT:
        return (
          wallet?.adapter.publicKey &&
          modal.lsuMint &&
          modal.maturityDate &&
          modal.lsuVault &&
          modal.userLsuAta &&
          modal.userPtAta && (
            <RedeemPTModal
              setModal={setModal}
              connection={connection}
              redeemer={wallet?.adapter.publicKey}
              lsuMint={modal.lsuMint}
              maturityDate={modal.maturityDate}
              lsuVault={modal.lsuVault}
              redeemerLsuAta={modal.userLsuAta}
              redeemerPtAta={modal.userPtAta}
            />
          )
        );
      case ModalState.ClaimYield:
        return (
          wallet?.adapter.publicKey &&
          modal.lsuMint &&
          modal.maturityDate &&
          modal.lsuVault &&
          modal.userLsuAta &&
          modal.userPtAta &&
          modal.userYtAta && (
            <ClaimYieldModal
              setModal={setModal}
              connection={connection}
              claimer={wallet?.adapter.publicKey}
              lsuMint={modal.lsuMint}
              maturityDate={modal.maturityDate}
              lsuVault={modal.lsuVault}
              claimerLsuAta={modal.userLsuAta}
              claimerYtAta={modal.userYtAta}
            />
          )
        );
    }
  };

  // The network can be set to 'devnet', 'testnet', or 'mainnet-beta'.
  const network = WalletAdapterNetwork.Devnet;

  // You can also provide a custom RPC endpoint.
  const endpoint = useMemo(() => clusterApiUrl(network), [network]);

  // @solana/wallet-adapter-wallets includes all the adapters but supports tree shaking and lazy loading --
  // Only the wallets you configure here will be compiled into your application, and only the dependencies
  // of wallets that your users connect to will be loaded.
  const wallets = useMemo(
    () => [
      //   new SolanaMobileWalletAdapter({
      //     appIdentity: { name: "Solana Wallet Adapter App" },
      //     authorizationResultCache: createDefaultAuthorizationResultCache(),
      //   }),
      new PhantomWalletAdapter(),
      //   new GlowWalletAdapter(),
      //   new SlopeWalletAdapter(),
      new SolflareWalletAdapter({ network }),
      new TorusWalletAdapter(),
    ],
    [network]
  );

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          <div className="spine">
            <div className="wallet-bar">
              <WalletMultiButton />
              <WalletDisconnectButton />
            </div>
            <div
              className={"mask" + (modal.state !== ModalState.None ? " visible" : "")}
              // onClick={() =>
              //   modal.state !== ModalState.None ? setModal({ state: ModalState.None }) : setMobileDropdownOpen(false)
              // }
            >
              {renderedModal()}
            </div>
            <Lysergic setModal={setModal} />
          </div>
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}
