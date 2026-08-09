import { Component, output } from '@angular/core';

@Component({
  selector: 'app-config',
  imports: [],
  templateUrl: './config.html',
  styleUrl: './config.css',
})
export class Config {
  close = output<void>();
}
