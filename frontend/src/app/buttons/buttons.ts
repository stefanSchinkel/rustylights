import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
@Component({
  selector: 'app-buttons',
  imports: [CommonModule],
  templateUrl: './buttons.html',
  styleUrl: './buttons.scss'
})
export class Buttons implements OnInit {
  notLoaded: boolean = true;
  constructor() { }

  ngOnInit() { }
}
