import { QRCode } from "@/registry/bases/opentui/ui/qr-code";

export default function QRCodeDemo() {
  return <QRCode value="https://rust-ui.com" size="md" label="Scan to visit" />;
}
