import { Component, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { HttpClient } from '@angular/common/http';
@Component({
  selector: 'app-buttons',
  imports: [CommonModule],
  templateUrl: './buttons.html',
  styleUrl: './buttons.scss',
})
export class Buttons implements OnInit {
  loaded: boolean = false;
  apiHost = "http://localhost:8000";
  data: any[] = [];

  constructor(
    private http: HttpClient) {
  }

  ngOnInit() {
    this.load_devices()
  }


  load_devices() {
    console.log("loading devices");
    this.http.get(this.apiHost + "/devices/").subscribe(
      reponse => { console.log(reponse); }
    )
    this.loaded = true;
    this.data = [
      { "id": 1, "name": "Button 1" },
      { "id": 2, "name": "Button 2" },
      { "id": 3, "name": "Button 3" }
    ]
  };
  onButtonClick(d: any) { console.log(d) };
  onCB(idx: number) { console.log("on for ", idx); }
  offCB(idx: number) { console.log("off for ", idx); }
}
