
export function BlogPostTOC({ headings }) {
	return (
		<div className="">
            {headings.map((section, i) => (
                <a key={i} href={`#${section.slug}`}>{section.text}</a>
            ))}
        </div>
	)
}