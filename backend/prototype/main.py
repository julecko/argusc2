import asyncio
import websockets

URI = "ws://127.0.0.1:3000/ws"
UUID_FILE = ".device_uuid"


def load_uuid() -> str:
    try:
        with open(UUID_FILE, "r") as f:
            return f.read().strip()
    except FileNotFoundError:
        return "new"


def save_uuid(uuid: str):
    with open(UUID_FILE, "w") as f:
        f.write(uuid)


async def main():
    my_uuid = load_uuid()
    print(f"Connecting with UUID: {my_uuid}")

    async with websockets.connect(URI) as ws:
        await ws.send(my_uuid)

        confirmed_uuid = await ws.recv()
        save_uuid(confirmed_uuid)
        print(f"Device UUID: {confirmed_uuid}")

        async for message in ws:
            print(f"Received: {message}")


if __name__ == "__main__":
    asyncio.run(main())