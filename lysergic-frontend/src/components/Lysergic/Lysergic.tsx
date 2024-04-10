import { useEffect, useState } from "react";
import "./Lysergic.scss";
import Table from "../table/Table";
import { ColType, ModalProps } from "../../types";

type Props = {
  setModal: React.Dispatch<React.SetStateAction<ModalProps>>;
};

function Lysergic(props: Props) {
  const { setModal } = props;

  const [bannerAnim, setBannerAnim] = useState("");

  const date = new Date();
  const twelve = new Date(date.setFullYear(date.getFullYear() + 1));
  const eighteen = new Date(date.setMonth(date.getMonth() + 18));
  const twentyfour = new Date(date.setFullYear(date.getFullYear() + 2));

  useEffect(() => {
    setBannerAnim("animate ");
  }, []);

  return (
    <div className="lysergic">
      <div className="banner">
        <h1 className={bannerAnim + "d1"}>L</h1>
        <h1 className={bannerAnim + "d2"}>Y</h1>
        <h1 className={bannerAnim + "d3"}>S</h1>
        <h1 className={bannerAnim + "d4"}>E</h1>
        <h1 className={bannerAnim + "d5"}>R</h1>
        <h1 className={bannerAnim + "d6"}>G</h1>
        <h1 className={bannerAnim + "d7"}>I</h1>
        <h1 className={bannerAnim + "d8"}>C</h1>
        <h1 className="blinker">_</h1>
      </div>
      <Table
        headers={[
          { label: "Vesting Date", type: ColType.Date, prefix: "", suffix: "" },
          { label: "Deposit Asset", type: ColType.String, prefix: "", suffix: "" },
          { label: "Stake Token", type: ColType.String, prefix: "", suffix: "" },
          { label: "Yield Token", type: ColType.String, prefix: "", suffix: "" },
          { label: "Actions", type: ColType.Actions, prefix: "", suffix: "" },
        ]}
        data={[
          [twelve.getTime().toString(), "MSOL", "PT-SOL", "YT-SOL", twelve.getTime().toString()],
          [eighteen.getTime().toString(), "MSOL", "PT-SOL", "YT-SOL", eighteen.getTime().toString()],
          [twentyfour.getTime().toString(), "MSOL", "PT-SOL", "YT-SOL", twentyfour.getTime().toString()],
        ]}
        alignRightLastCol={false}
        colClasses={["fifth", "fifth", "fifth", "fifth", "fifth"]}
        setModal={setModal}
      />
    </div>
  );
}

export default Lysergic;
