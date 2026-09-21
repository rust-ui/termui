import { QRCode } from "@/registry/bases/ink/ui/qr-code";

export default function QRCodeDemo() {
  return (
    <QRCode
      value="https://rust-ui.com"
      size="md"
      label="Scan to visit"
      alt="Open rust-ui.com"
    />
  );
}
