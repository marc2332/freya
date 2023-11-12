export function BlogPostTOC({ headings }) {
  return (
    <div className="w-1/5 h-screen overflow-auto sticky top-5 mx-4 text-sm hidden sm:block">
      {headings.map((section, i) => (
        <a
          key={i}
          href={`#${section.slug}`}
          className="block hover:bg-zinc-800 p-2 rounded-xl cursor-pointer"
          style={{ marginLeft: `${(section.depth - 2) * 10}px` }}
        >
          {section.text} <br />
        </a>
      ))}
    </div>
  );
}
