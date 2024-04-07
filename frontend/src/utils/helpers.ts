import { validInputs } from "../types/input";
import { PlayerId, ProtobufType, ServerOutput } from "../types/responses";
import proto from "protobufjs";

export function isValidInput(input: string): input is keyof typeof validInputs {
  return input in validInputs;
}

export function typeOfMessage(decoded: proto.Message<{}>): ProtobufType {
  console.log(decoded);
  const { type } = { ...(decoded as unknown as { type: number }) };

  console.log(type);

  switch (type) {
    case 1:
      return ProtobufType.Frame;
    case 2:
      return ProtobufType.IdResponse;
    case 3:
      return ProtobufType.InputRequest;
  }
  throw Error("Invalid input to this function");
}

export function makeDecodedObjectIntoNiceTypescriptObject(
  decoded: proto.Message<{}>,
  protobufType: ProtobufType
): ServerOutput | PlayerId {
  switch (protobufType) {
    case ProtobufType.Frame: {
      const serverOutput = { ...(decoded as unknown as ServerOutput) };
      return serverOutput;
    }
    case ProtobufType.IdResponse: {
      const playerId = { ...(decoded as unknown as PlayerId) };
      return playerId;
    }
  }
  throw Error(
    "Shouldn't be using this to anything else. And please parse before using!"
  );
}
