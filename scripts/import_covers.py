import time
import urllib.parse

from bs4 import BeautifulSoup
import requests
from requests.adapters import HTTPAdapter


def main():
    with requests.Session() as session:
        # Retry ConnectionError
        session.mount("https://mangadex.org", HTTPAdapter(max_retries=5))
        items = session.get("http://localhost:8088/item").json()
        for item in items:
            if item["status"] == "InProgress":
                continue
            if not item["extra"]:
                continue
            if "mangadex_url" not in item["extra"]:
                continue
            if item["covers"]:
                print("Item has covers", item["covers"])
                continue

            url = item["extra"]["mangadex_url"]
            root = BeautifulSoup(session.get(url).text, "html.parser")
            covers_url = next((link for link in root.select("a") if link.attrs["href"].endswith("/covers/")), None)
            if not covers_url:
                continue
            covers_url = urllib.parse.urljoin(url, covers_url.attrs["href"])
            print("Fetching covers for", item["key"], "from", covers_url)

            root = BeautifulSoup(session.get(covers_url).text, "html.parser")
            images = []
            for img in root.select(".tab-content img"):
                url = img.parent.attrs["href"]
                description = img.attrs["alt"]
                print("Fetching", url, ":", description)
                resp = session.get(url)
                images.append((description, resp.content, resp.headers["Content-Type"]))
                time.sleep(3)

            if not images:
                print("No covers found")
                continue

            print("Total cover size:", sum(len(i[1]) for i in images) / (2 ** 20), "MiB")

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

            print("Waiting 10 seconds...")
            time.sleep(10)


if __name__ == "__main__":
    main()
