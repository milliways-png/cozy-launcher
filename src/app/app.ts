import { Component, signal, NgZone } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { getCurrentWindow } from '@tauri-apps/api/window'

@Component({
  selector: 'app-root',
  imports: [RouterOutlet],
  templateUrl: './app.html',
  styleUrl: './app.css'
})
export class App {
  constructor(private ngZone: NgZone) {}

  protected readonly title = signal('cozy-launcher');
  
  onWindowDrag(event: MouseEvent) {
    // Only drag if the user clicks the left mouse button
    if (event.button === 0) {
      // Run outside Angular change detection for instant native response
      this.ngZone.runOutsideAngular(async () => {
        try {
          const appWindow = getCurrentWindow();
          await appWindow.startDragging();
        } catch (error) {
          console.error("Tauri drag failed:", error);
        }
      });
    }
  }  
}
