import type { IdentifiableInterface } from "../identifiable-interface.ts";

export interface FileInterface extends IdentifiableInterface {
  name: string;
  mime: string;
  path: string;
}

type FileCreateUri = { path: string } | { url: string };

export interface FileCreateInterface {
  name: string;
  uri: FileCreateUri;
}

export interface FileUpdateInterface extends IdentifiableInterface {
  name?: string;
}
