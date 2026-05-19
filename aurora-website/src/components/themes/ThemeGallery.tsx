"use client";

import { useDeferredValue, useState } from "react";
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
import { Input } from "@/components/ui/input";
import { type Theme, getThemeSlug } from "@/data/themes";

type ThemeGalleryProps = {
  themes: Theme[];
};

export function ThemeGallery({ themes }: ThemeGalleryProps) {
  const [query, setQuery] = useState("");
  const deferredQuery = useDeferredValue(query);
  const normalizedQuery = deferredQuery.trim().toLowerCase();

  const filteredThemes = themes.filter((theme) => {
    if (!normalizedQuery) {
      return true;
    }

    return theme.name.toLowerCase().includes(normalizedQuery);
  });

  return (
    <div className="flex flex-col gap-6">
      <div className="max-w-md">
        <Input
          type="search"
          value={query}
          onChange={(event) => setQuery(event.target.value)}
          placeholder="Search themes..."
          aria-label="Search themes"
          className="bg-card"
        />
      </div>

      {filteredThemes.length > 0 ? (
        <div className="grid gap-6 md:grid-cols-2 xl:grid-cols-3">
          {filteredThemes.map((theme) => (
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
      ) : (
        <Card className="p-6">
          <p className="text-sm text-muted-foreground">
            No themes match "{query.trim()}".
          </p>
        </Card>
      )}
    </div>
  );
}
