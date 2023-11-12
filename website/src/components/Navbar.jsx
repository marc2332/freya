export function Navbar() {
  return (
    <div className="mx-auto my-2  text-stone-300 w-full p-4 sm:py-8 px-4 bg-custom-dark-black">
      <div className="w-full flex justify-center items-center  sm:hidden pb-6">
        <a className="mx-4 hover:text-white" title="Freya Home Page" href="/">
          <img src="/logo.svg" width="40" className="inline-block " />
          <img src="/freya.svg" width="50" className="ml-4 inline-block " />
        </a>
      </div>
      <div className="flex items-center">
        <a
          className="mx-4 hover:text-white static top-0 hidden sm:inline-block "
          title="Freya Home Page"
          href="/"
        >
          <img src="/logo.svg" width="40" className="inline-block " />
          <img src="/freya.svg" width="50" className="ml-4 inline-block " />
        </a>
        <a
          className="inline-block mx-4 hover:text-white hover:underline"
          title="Freya Blog"
          href="/blog"
        >
          Blog
        </a>
        <a
          className="inline-block mx-4 hover:text-white hover:underline"
          title="Book Book"
          href="https://book.freyaui.dev/"
        >
          Book
        </a>
        <a
          className="inline-block mx-4 hover:text-white hover:underline"
          title="Freya Source Code"
          href="https://github.com/marc2332/freya"
        >
          Contribute
        </a>
        <a
          className="inline-block mx-4 hover:text-white hover:underline"
          title="Freya Discord"
          href="https://discord.gg/sYejxCdewG"
        >
          Discord
        </a>
      </div>
    </div>
  );
}
