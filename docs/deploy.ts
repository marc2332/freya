import { serve } from "https://deno.land/std@0.140.0/http/server.ts";
import { parse } from "https://deno.land/std@0.176.0/path/mod.ts";

async function handleRequest(request: Request) {
  const url = new URL(request.url);
  let filepath = decodeURIComponent(url.pathname);
  const parsedFilePath = parse(filepath);

  let file;
  try {
    let newFilepath = filepath;
    if (parsedFilePath.ext == "") {
      newFilepath += ".html";
    }
    file = await Deno.open("./book" + newFilepath, { read: true });
  } catch {
    try {
      file = await Deno.open("./book" + filepath + "index.html", {
        read: true,
      });
    } catch {
      file = await Deno.open("./book/404.html", { read: true });
    }
  }

  return new Response(file.readable);
}

serve(handleRequest);
