import { ApiPromise, WsProvider } from "@polkadot/api";

const WS_URL = "ws://localhost:9944";
const connection = async () => {

  const wsProvider = new WsProvider(WS_URL);
  const api = await ApiPromise.create({ provider: wsProvider });
  await api.isReady;
  return api;
}

const getevents = async (api: ApiPromise) => {
  await api.query.system.events((events: any[]) =>{
    events.forEach((event) => {
      console.log("index: ", event['event']['index'].toHuman());
      console.log("data: ", event['event']['data'].toHuman());
    })
  });
}

const main =async () => {
    const api = await connection();
    await getevents(api);
    await sleep(100000);
  console.log("main!");
}

main().then(() => {
  console.log("exits successfully!");
  process.exit(0);
}).catch((error) => {  
  console.error("error is ",error);
  process.exit(1);
});

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));
