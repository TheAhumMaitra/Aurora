import { Button } from "../ui/button";
import Aurora from "../Aurora";
import Link from "next/link";

export default function Hero() {
  return (
    <div className="flex flex-col justify-center h-90 items-center gap-3">
      <Aurora
        colorStops={["#0d7328", "#30ff67", "#00d93a"]}
        blend={0.5}
        amplitude={1.0}
        speed={1}
      />

      <h1 className="text-4xl font-bold">Meet Aurora!</h1>
      <p className="text-2sm ml-7 wrap-normal mr-7">
        A elegant, minimal, aesthetic, Rust based rice!
      </p>
      <div className="flex justify-between items-center gap-3">
        <Button className="p-5 font-bold text-sm cursor-pointer">
          <Link href="https://aurorawiki.vercel.app">
            Wiki
          </Link>
        </Button>
        <Button className="p-5 cursor-pointer bg-destructive hover:bg-chart-2 font-bold text-sm">
          <Link
            href="https://github.com/TheAhumMaitra/Aurora.git"
          >
            Github
          </Link>
        </Button>
      </div>
    </div>
  );
}
