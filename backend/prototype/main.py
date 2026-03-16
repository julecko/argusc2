import asyncio
import websockets
 
URI = "ws://127.0.0.1:3000/ws"
 
async def main():
    async with websockets.connect(URI) as ws:
        print(f"Connected to {URI}")
 
        # Send a message
        await ws.send("Hello from Python")
 
        # Listen for messages
        async for message in ws:
            print(f"Received: {message}")
 
if __name__ == "__main__":
    asyncio.run(main())
 