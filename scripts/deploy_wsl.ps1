#!/mnt/c/Windows/System32/WindowsPowerShell/v1.0/powershell.exe



if (-not (Test-Path d:\)) {
    echo "d:\ not mounted?"
    exit 1
}
cp .\target\x86_64-unknown-uefi\debug\rustyvisor.efi d:\
cp .\target\x86_64-unknown-uefi\debug\rustyvctl.efi d:\
# https://serverfault.com/questions/130887/dismount-usb-external-drive-using-powershell
$driveEject = New-Object -comObject Shell.Application;
$driveEject.Namespace(17).ParseName("D:").InvokeVerb("Eject");

echo "ejecting"
while (Test-Path d:\ -ErrorAction SilentlyContinue) {
    Start-Sleep -Milliseconds 500
}
echo "ejected"
