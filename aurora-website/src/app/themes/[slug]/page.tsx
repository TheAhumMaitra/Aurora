import type { Metadata } from "next";
import Image from "next/image";
import Link from "next/link";
import { notFound } from "next/navigation";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  getThemeBySlug,
  getThemeSlug,
  themes,
} from "@/data/themes";
import {
  Table,
  TableBody,
  TableCell,
  TableRow,
} from "@/components/ui/table";

export function generateStaticParams() {
  return themes.map((theme) => ({ slug: getThemeSlug(theme.name) }));
}

export async function generateMetadata({
  params,
}: {
  params: Promise<{ slug: string }>;
}): Promise<Metadata> {
  const { slug } = await params;
  const theme = getThemeBySlug(slug);

  if (!theme) {
    return {
      title: "404 Theme Not Found",
    };
  }

  return {
    title: `${theme.name} | Aurora Themes`,
    description: `${theme.name} theme details and status.`,
  };
}

export default async function ThemeDetailsPage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug } = await params;
  const theme = getThemeBySlug(slug);

  if (!theme) {
    notFound();
  }

  return (
    <main className="flex-1 px-4 py-10">
      <section className="mx-auto flex w-full max-w-6xl flex-col gap-8">
        <div className="flex flex-wrap items-center justify-between gap-4">
          <div className="space-y-3">
            <Link
              href="/themes"
              className="text-xs uppercase tracking-[0.3em] text-primary"
            >
              Back To Themes
            </Link>
            <div className="flex flex-wrap items-center gap-3">
              <h1 className="font-heading text-4xl font-black uppercase">
                {theme.name}
              </h1>
              <Badge
                variant={theme.status === "official" ? "default" : "secondary"}
              >
                {theme.status}
              </Badge>
            </div>
          </div>

          <Button asChild variant="outline" className="uppercase bg-background p-3 rounded-2xl">
            <a href={theme.repoUrl} target="_blank" rel="noreferrer">
              Visit repository
            </a>
          </Button>
        </div>

        <div className="grid gap-6 lg:grid-cols-[1.4fr_0.8fr]">
          <Card className="overflow-hidden">
            <Image
              src={theme.previewImageUrl}
              alt={`${theme.name} preview`}
              width={1200}
              height={720}
              sizes="(min-width: 1024px) 60vw, 100vw"
              quality={100}
              className="aspect-5/3 h-auto w-full object-cover"
            />
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Theme Details</CardTitle>
            </CardHeader>
            <CardContent>
              <Table>
                <TableBody>
                  <TableRow>
                    <TableCell className="font-semibold">Name</TableCell>
                    <TableCell>{theme.name}</TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell className="font-semibold">Status</TableCell>
                    <TableCell className="uppercase">{theme.status}</TableCell>
                  </TableRow>
                </TableBody>
              </Table>
            </CardContent>
          </Card>
        </div>
      </section>
    </main>
  );
}
