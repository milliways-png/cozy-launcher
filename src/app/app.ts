import { Component, signal, NgZone, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { invoke } from '@tauri-apps/api/core';
import { RouterOutlet } from '@angular/router';
import { getCurrentWindow } from '@tauri-apps/api/window'

@Component({
  selector: 'app-root',
  imports: [RouterOutlet, CommonModule],
  templateUrl: './app.html',
  styleUrl: './app.css'
})
export class App implements OnInit {
  shortcutFiles: string[] = [];
  protected readonly title = signal('cozy-launcher');

  constructor(private ngZone: NgZone) {}

  ngOnInit(): void {
    this.loadShortcuts();
  }

  // NOTE: Ask Rust for the list of files inside "my_app_data"
  async loadShortcuts() {
    try {
      this.shortcutFiles = await invoke<string[]>('get_shortcuts');      
    } catch (error) {
      console.log('could not read shortcuts folder', error);
    }
  }

  // NOTE: Tell Rust to launch the file that was clicked
  async openShortcut(filename: string) {
    try {
      await invoke('launch_shortcut', { filename });
    } catch (error) {
      console.error('could not execute ${filename}:', error);
    }
  }

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
