import { PublicKey } from "@solana/web3.js";
import { Numberu64 } from "../hooks/api";

//login
export type Authentication = {
  authorization: string;
};

//UI
export enum ColType {
  String = 0,
  Number = 1,
  Date = 2,
  Actions = 3,
}

export type ModalProps = {
  state: ModalState;
  lsuMint?: PublicKey;
  maturityDate?: Date;
  lsuVault?: PublicKey;
  userLsuAta?: PublicKey;
  userPtAta?: PublicKey;
  userYtAta?: PublicKey;
  amount?: Numberu64;
};

export enum ModalState {
  None = 0,
  TokenizeYield = 1,
  RedeemYield = 2,
  RedeemFromPT = 3,
  ClaimYield = 4,
}

//user balances
export type Balances = {
  [accountName: string]: AccountBalance;
};

export type AccountBalance = {
  account_name: string;
  perp: Perp[];
  spot: Spot[];
};

export type Perp = {
  asset: string;
  available_balance: number;
  balance: number;
  cross_unrealized_profit: number;
  cross_wallet_balance: number;
  unrealized_profit: number;
};

export type Spot = {
  asset: string;
  free: number;
  locked: number;
};

//user positions
export type Positions = {
  [accountName: string]: Position[];
};

export type Position = {
  symbol: string;
  amount: string;
  entry_price: string;
  mark_price: string;
  margin_type: string;
  leverage: string;
  liquidation_price: string;
  unrealized_profit: string;
  side: string;
  account_name?: string;
};

//config
export type Config = {
  exchange_apikey_fields: {
    [field: string]: string[];
  };
  order_option: {
    algo_hidden_price_option: {
      [option: string]: string[];
    };
    exchange_to_symbol: {
      [ex_to_sym: string]: string[];
    };
    order_sides: string[];
    order_types: string[];
    product_types: string[];
  };
};

//orders
export type Order = {
  account_nick_name: string;
  //v this is ? but let's not fuck with it for now
  algo_extra: {
    hidden_order_info: {
      actual_trigger_price: number;
      ref_exchange: string;
      trigger_price: string;
    };
  };
  algorithm: string;
  create_time: number;
  error_reason: string;
  exchange: string;
  exchange_order_id: string;
  exec_time: number;
  finish_time: number;
  order_id: string;
  order_type: string;
  price: string;
  price_filled: string;
  product_type: string;
  side: string;
  size: string;
  size_filled: string;
  state: string;
  symbol: string;
  user_email: string;
};

export type OrderForm = {
  market_account_name: string;
  market_name: string; //for now
  symbol: string;
  order_type: string;
  side: "BUY" | "SELL" | "";
  price: string;
  size: string;
  product_type: string;
  ref_exchange?: string; //for now
  trigger_price?: string;
};

//market accounts
export type MarketAccount = {
  api_key: string;
  create_time: number;
  market_name: string;
  nickname: string;
};

//websocket message

export type SocketStack = {
  user_email: string;
  type: "account_balance" | "perp_position_update" | "order_update";
  data: unknown;
  account_name: string;
};
