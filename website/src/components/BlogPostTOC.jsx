
export function BlogPostTOC({ headings }) {
	return (
		<div className="w-1/5 h-screen overflow-auto block sticky top-2 mx-4 text-sm">
            {headings.map((section, i) => (
                 <a key={i} href={`#${section.slug}`} className="block hover:bg-stone-700 p-2 rounded-xl cursor-pointer" style={{"margin-left": `${(section.depth - 2)* 10}px`}}>

                                {section.text} <br/>
  
                   </a>
                    
                
            ))}
        </div>
	)
}