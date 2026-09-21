import { QRCode } from "@/registry/bases/opentui/ui/qr-code";

export default function QRCodeDemo() {
  return <QRCode value="https://termui.rustify.app" size="md" label="Scan to visit" />;
}
