export type CommandError =
  | { Db: string }
  | { OcrInitialize: string }
  | { OcrSetImage: string }
  | { OcrGetHocrText: string }
  | { Tauri: string }
  | { Anyhow: string }
  | { ExternalRecipeUrlNotSupported: string }
  | { NotFound: string };
