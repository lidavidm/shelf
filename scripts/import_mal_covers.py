import time
import urllib.parse

from bs4 import BeautifulSoup
import requests
from requests.adapters import HTTPAdapter


def main():
    with requests.Session() as session:
        # Retry ConnectionError
        session.mount("https://myanimelist.net", HTTPAdapter(max_retries=5))
        items = session.get("http://localhost:8088/item").json()
        for item in items:
            if item["kind"] != "Manga":
                continue
            if not item["extra"]:
                continue
            if "mal_id" not in item["extra"]:
                continue
            if "external_url" in item["extra"]:
                continue
            if "mangadex_url" in item["extra"]:
                continue
            if item["covers"]:
                continue
            print(item["extra"])

            url = f"https://myanimelist.net/manga/{item['extra']['mal_id']}"
            root = BeautifulSoup(session.get(url).text, "html.parser")
            links = root.select('#content a')
            cover = None
            for link in links:
                if link.attrs.get("href", "").endswith("pics"):
                    img = link.find("img")
                    if not img:
                        continue
                    cover = img.attrs["data-src"]
            if not cover:
                print("Could not find cover for", item["key"])
                continue
            cover = cover.replace(".jpg", "l.jpg")
            print("Fetching", cover)
            resp = session.get(cover)
            images = [("Cover", resp.content, resp.headers["Content-Type"])]
            files = {}
            covers = []
            for i, (description, contents, mime_type) in enumerate(images):
                blob_name = f"blob-{item['key']}-{i:03}"
                files[blob_name] = (blob_name, contents, mime_type)
                covers.append((blob_name, description))
            print("Uploading covers...")
            print(session.put("http://localhost:8088/blob", files=files).json())
            item["covers"] = covers
            print(session.post(f"http://localhost:8088/item/{item['key']}", json=item).json())
            time.sleep(5)


if __name__ == "__main__":
    main()
