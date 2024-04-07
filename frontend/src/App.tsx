import GameWindow from "./components/GameWindow";
import { validInputs } from "./types/input";
import {
  isValidInput,
  makeDecodedObjectIntoNiceTypescriptObject,
  typeOfMessage,
} from "./utils/helpers";
import { useEffect, useRef, useState } from "react";
import isEqual from "lodash/isEqual";
import proto from "protobufjs";
import { Frame, PlayerId, ProtobufType, ServerOutput } from "./types/responses";

function App() {
  const [keysDown, setKeysDown] = useState<Set<number>>(new Set());
  const [sentInputs, setSentInputs] = useState<number[]>([]);
  const [id, setId] = useState(0);
  const [frame, setFrame] = useState<Frame[]>([]);
  const connection = useRef<WebSocket | null>(null);
  const protoRoot = useRef<proto.Root | null>(null);

  function giveMeTheRightProtoForTheJob(
    protobufType: ProtobufType
  ): proto.Type | undefined {
    if (!protoRoot.current) {
      return;
    }
    switch (protobufType) {
      case ProtobufType.Frame:
        return protoRoot.current.lookupType("ServerOutput");
      case ProtobufType.IdResponse:
        return protoRoot.current.lookupType("PlayerId");
      case ProtobufType.InputRequest:
        return protoRoot.current.lookupType("InputRequest");
    }
  }

  function handleKeyDown(event: KeyboardEvent) {
    event.preventDefault();
    if (!isValidInput(event.code)) {
      return;
    }

    const parsedInput = validInputs[event.code]();
    setKeysDown((keys) => new Set([...keys, parsedInput]));
  }

  function handleKeyUp(event: KeyboardEvent) {
    event.preventDefault();
    if (!isValidInput(event.code)) {
      return;
    }

    const parsedInput = validInputs[event.code]();
    setKeysDown((prevKeys) => {
      const updatedSet = new Set(prevKeys);
      updatedSet.delete(parsedInput);
      return updatedSet;
    });
  }

  // Send users inputs from keysDown to backend for processing
  useEffect(() => {
    const keysArray = Array.from(keysDown);
    const inputProto = giveMeTheRightProtoForTheJob(ProtobufType.InputRequest);

    if (
      !connection.current ||
      !inputProto ||
      id == 0 ||
      isEqual(keysArray, sentInputs)
    ) {
      return;
    }
    const payload = {
      playerId: id,
      input: keysArray.reduce((a, b) => a + b, 0),
    };

    console.log(payload);

    const message = inputProto.create(payload);

    if (!message) {
      throw Error("Failed to create the message");
    }
    const buffer = inputProto.encode(message).finish();

    if (!buffer) {
      throw Error("Failed to encode message to binary");
    }
    connection.current.send(buffer);
    setSentInputs(keysArray);
  }, [keysDown]);

  useEffect(() => {
    proto.load("messages.proto", (err, root) => {
      if (err) {
        console.log(err);
      }
      protoRoot.current = root || null;
    });

    const socket = new WebSocket("ws://127.0.0.1:9999");
    socket.addEventListener("open", () => {
      console.log("We are open!");
    });

    socket.addEventListener("error", (e) => {
      console.log("things went south  ", e);
    });

    socket.addEventListener("message", async (event): Promise<void> => {
      const data = event.data as Blob;
      const uintArr = new Uint8Array(await data.arrayBuffer());

      let proto = giveMeTheRightProtoForTheJob(
        id === 0 ? ProtobufType.IdResponse : ProtobufType.Frame
      );

      let decoded = proto?.decode(uintArr);
      console.log(decoded);
      if (!decoded) {
        return;
      }
      const messageType = typeOfMessage(decoded!);
      proto = giveMeTheRightProtoForTheJob(messageType);

      if (messageType === ProtobufType.IdResponse) {
        const playerId = makeDecodedObjectIntoNiceTypescriptObject(
          decoded,
          messageType
        ) as PlayerId;
        setId(playerId.playerId);
        return;
      }

      if (messageType === ProtobufType.Frame) {
        decoded = proto?.decode(uintArr);
        const { frames } = {
          ...(makeDecodedObjectIntoNiceTypescriptObject(
            decoded!,
            messageType
          ) as ServerOutput),
        };

        setFrame(frames);
        return;
      }
    });

    document.addEventListener("keydown", handleKeyDown);
    document.addEventListener("keyup", handleKeyUp);

    connection.current = socket;

    return () => {
      socket.close();
      document.removeEventListener("keydown", handleKeyDown);
      document.removeEventListener("keyup", handleKeyUp);
    };
  }, []);

  return <GameWindow frame={frame} />;
}

export default App;
