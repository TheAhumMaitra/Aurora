import { ThemeGallery } from "@/components/themes/ThemeGallery";
import { themes } from "@/data/themes";

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
        <ThemeGallery themes={themes} />
      </section>
    </main>
  );
}
