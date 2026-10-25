// Read-only native macOS CoreGraphics window witness for Nexora Code Beta.
// Never enumerates other apps' titles, screenshots, controls or user data.
import Foundation
import CoreGraphics
guard CommandLine.arguments.count == 2,
      let requestedPID = Int32(CommandLine.arguments[1]), requestedPID > 1 else {
    fputs("BLOCKED: expected a positive test-app PID\\n", stderr)
    exit(3)
}
guard let rows = CGWindowListCopyWindowInfo([.optionOnScreenOnly, .excludeDesktopElements],
                                           kCGNullWindowID) as? [[String: Any]] else {
    fputs("BLOCKED: cannot enumerate native window metadata\\n", stderr)
    exit(3)
}
var visibleCount = 0
for window in rows {
    guard (window[kCGWindowOwnerPID as String] as? Int32) == requestedPID else { continue }
    let layer = window[kCGWindowLayer as String] as? Int ?? -1
    guard layer == 0 else { continue }
    guard let dictionary = window[kCGWindowBounds as String] as? [String: CGFloat] else { continue }
    let width = dictionary["Width"] ?? 0
    let height = dictionary["Height"] ?? 0
    if width >= 400 && height >= 300 { visibleCount += 1 }
}
if visibleCount > 0 {
    print("PASS: test PID owns \(visibleCount) on-screen native application window(s); React contents not verified")
    exit(0)
}
print("BLOCKED: no visible native window for exact beta PID; no GUI PASS claimed")
exit(3)
