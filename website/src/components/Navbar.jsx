
export function Navbar() {
	return (
		<div className="top-0 sticky mx-auto my-6 flex items-center text-stone-300 ">
            <div className="w-full py-8 px-4 bg-custom-dark-black">
            <a className="mx-4 hover:text-white inline-block mr-8" title="Freya Home Page" href="/">
                <img src="/logo.svg" width="40" className="inline-block"/>
                <img src="/freya.svg" width="50" className="ml-4 inline-block"/>
            </a>
            <a className="mx-4 hover:text-white hover:underline" title="Freya Blog" href="/posts">Blog</a>
            <a className="mx-4 hover:text-white hover:underline" title="Book Book" href="https://book.freyaui.dev/">Book</a>
            <a className="mx-4 hover:text-white hover:underline" title="Freya Source Code" href="https://github.com/marc2332/freya" class="inline-block m-2">Contribute</a>
			<a className="mx-4 hover:text-white hover:underline" title="Freya Discord" href="https://discord.gg/sYejxCdewG" class="inline-block m-2">Discord</a>
            </div>
        </div>
	)
}