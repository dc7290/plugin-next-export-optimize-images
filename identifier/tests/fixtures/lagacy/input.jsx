import Image from "next/legacy/image";
import src from "./test.png";

const Component = () => {
  return (
    <div>
      <Image
        src="https://sample.com/images/test.png"
        width={500}
        height={500}
      />
      <Image src="/image.png" width={500} height={500} />
      <Image src={src} sizes="100vw" />
    </div>
  );
};

export default Component;
