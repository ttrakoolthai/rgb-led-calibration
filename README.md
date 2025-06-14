# RGB LED Calibration

**Author:** Tommy Trakoolthai
**Assignment:** June 11, 2025

---

## What I Did

This project adjusts the brightness of the red, green, and blue LEDs on the MicroBit v2 to help calibrate them for a balanced white light. It uses an external rotary knob along with the MicroBit’s built-in buttons to control the settings:
	•	Pressing Button A changes the blue brightness
	•	Pressing Button B changes the green brightness
	•	Pressing A and B together changes the red brightness
	•	When no buttons are pressed, the rotary knob adjusts the frame rate

The MicroBit prints the current RGB values and frame rate to the terminal in order to monitor changes and fine tune the calibration.
---

## Calibration Results

I tested many color combinations by adjusting the red, green, and blue levels. This included switching between full white (all colors on) and toggling individual colors on and off rapidly. The system handled these transitions reliably.

For nearly all tests, the frame rate remained steady at 160 FPS, which kept the LEDs looking smooth and consistent. In a few rare cases—especially when switching colors quickly—the frame rate briefly dropped to 10 FPS, but it always recovered without any lasting issues.

---

## Calibration Summary

  • A balanced white color was achieved with: red: 15, green: 15, blue: 15
	•	The system maintained a stable frame rate of 160 FPS
	•	Green and blue LEDs were active for approximately 94% of each frame
	•	The output appeared bright and steady under normal use; only rapid color switching occasionally caused a brief flicker

---

## Output Snippet
```
red: 15
green: 15
blue: 15
frame rate: 160

red: 15
green: 0
blue: 15
frame rate: 160

red: 15
green: 15
blue: 0
frame rate: 160

red: 0
green: 15
blue: 15
frame rate: 160

red: 15
green: 15
blue: 15
frame rate: 10

red: 15
green: 15
blue: 15
frame rate: 160
```

---

## Testing and Verification

I used the RTT terminal to check that the RGB LED values and frame rate were updating correctly. Whenever I changed the buttons or the knob, the system printed the new values in real time. This let me confirm that:

- Button A changed the blue brightness
- Button B changed the green brightness
- Pressing both buttons changed the red brightness
- Turning the knob (with no buttons pressed) changed the frame rate

---

## Demo

See PHOTO.jpg in the ZIP archive for a picture of the working setup, or check the video linked in VIDEO_LINK.txt.

---

## Attribution

This project is based on code provided by Bart Massey, available at:
https://github.com/pdx-cs-rust-embedded/hw-rgbcal-skeleton

ChatGPT was used to help understand the code and to help clean up the wording used in documentation.
