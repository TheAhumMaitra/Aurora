import Image from "next/image";
import Link from "next/link";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { themes, getThemeSlug } from "@/data/themes";

export default function ThemesPage() {
  return (
    <main className="flex-1 px-4 py-10">
      <section className="mx-auto flex w-full max-w-6xl flex-col gap-8">
        <div className="space-y-3">
          <p className="text-xs uppercase tracking-[0.3em] text-primary">
            Theme Library
          </p>
          <h1 className="font-heading text-4xl font-black uppercase">
            All Themes
          </h1>
          <p className="max-w-2xl text-sm text-muted-foreground">
            All gorgeous themes you can add on your Aurora!
          </p>
        </div>

        <div className="grid gap-6 md:grid-cols-2 xl:grid-cols-3">
          {themes.map((theme) => (
            <Card key={theme.id} className="overflow-hidden">
              <Link href={`/themes/${getThemeSlug(theme.name)}`}>
                <Image
                  src={theme.previewImageUrl}
                  alt={`${theme.name} preview`}
                  width={1200}
                  height={720}
                  sizes="(min-width: 1280px) 384px, (min-width: 768px) 50vw, 100vw"
                  quality={100}
                  className="aspect-[5/3] h-auto w-full border-b-2 border-border object-cover"
                />
              </Link>

              <CardHeader>
                <div className="flex items-start justify-between gap-3">
                  <div className="space-y-1">
                    <CardTitle>{theme.name}</CardTitle>
                  </div>
                  <Badge
                    variant={
                      theme.status === "official" ? "default" : "secondary"
                    }
                  >
                    {theme.status}
                  </Badge>
                </div>
              </CardHeader>

              <CardFooter>
                <Button asChild className="w-full uppercase">
                  <Link href={`/themes/${getThemeSlug(theme.name)}`}>
                    Visit Theme
                  </Link>
                </Button>
              </CardFooter>
            </Card>
          ))}
        </div>
      </section>
    </main>
  );
}
