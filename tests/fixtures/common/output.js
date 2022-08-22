import { registerImage } from "next-export-optimize-images/api";
import Image from "next/image";

const Component = () => {
  return <Image src="/img.png" width={1280} height={960} alt="" />;
};

export default Component;

registerImage({ src: "/img.png", width: 1280, height: 960, alt: "" });
