export async function getDataUrl(file: File): Promise<string> {
  const reader = new FileReader();
  return new Promise((resolve) => {
    reader.addEventListener("load", () => {
      resolve(reader.result as string);
    });
    reader.readAsDataURL(file);
  });
}
