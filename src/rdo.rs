use bitfield::bitfield;

bitfield! {
  pub struct Rdo(u32);
  impl Debug;
  // The fields default to u16
  pub position, _: 30, 28;
  pub give_back, _ : 27;
  pub capability_mismatch, _: 26;
  pub usb_communication_capable, _: 25;
  pub no_usb_suspend, _: 24;
  pub unchunked_extended_messages, _: 23;
  pub operating_current, _: 19, 10;
  pub max_operating_current, _: 9, 0;
}
