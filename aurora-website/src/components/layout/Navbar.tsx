"use client";

import { usePathname } from "next/navigation";
import Link from "next/link";
import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import { GiHamburgerMenu } from "react-icons/gi";

import { Sheet, SheetContent, SheetTrigger } from "@/components/ui/sheet";

const navItems = [
  { name: "Home", href: "/" },
  { name: "Themes", href: "/themes" },
  { name: "Wiki", href: "https://aurorawiki.vercel.app/" },
];
function MobileNavbar() {
  const pathname = usePathname();
  return (
    <Sheet>
      <SheetTrigger asChild>
        <Button className="p-2">
          <GiHamburgerMenu className="scale-150" />
        </Button>
      </SheetTrigger>
      <SheetContent>
        <ul className="flex flex-col justify-center items-center gap-3 text-lg m-3">
          {navItems.map((link) => {
            const isActive = pathname === link.href;
            return (
              <Link
                key={link.name}
                href={link.href}
                className={cn(
                  "text-foreground hover:bg-primary hover:rounded-2xl hover:p-1  transition delay-150 duration-200 ease-in-out hover:-translate-y-0.5 ",
                  isActive && "text-primary font-bold border-3 p-1 rounded-2xl",
                )}
              >
                {link.name}
              </Link>
            );
          })}
        </ul>
      </SheetContent>
    </Sheet>
  );
}

export default function Navbar() {
  const pathname = usePathname();
  return (
    <nav className="flex justify-between p-2 items-center border-4 m-1 rounded-2xl border-border">
      <h1 className="text-xl font-extrabold">Aurora</h1>
      <ul className="hidden lg:flex justify-center items-center gap-3">
        {navItems.map((link) => {
          const isActive = pathname === link.href;
          return (
            <Link
              key={link.name}
              href={link.href}
              className={cn(
                "text-foreground hover:bg-primary hover:rounded-2xl hover:p-1  transition delay-150 duration-200 ease-in-out hover:-translate-y-0.5 ",
                isActive && "text-primary font-bold border-3 p-1 rounded-2xl",
              )}
            >
              {link.name}
            </Link>
          );
        })}
      </ul>
      <span className="lg:hidden">
        <MobileNavbar />
      </span>
    </nav>
  );
}
