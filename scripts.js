
document.body.addEventListener('keydown', navigate, false);
window.onload = function() { window.location.hash = '#slide0' }

function navigate(event) {
  var slideCount = document.querySelectorAll('section').length;
  var activeSlide = document.querySelector('[id^="slide"]:target');
  var slideNum = parseInt(activeSlide.getAttribute('id').substring(5));

  var requestedNext = event.code === 'Space' || event.code === 'ArrowRight';
  var requestedPrev = event.code === 'ArrowLeft';

  if (activeSlide && slideNum < slideCount - 1 && requestedNext) {
    window.location.hash = 'slide' + (slideNum + 1);
  }

  if (slideNum > 0 && requestedPrev) {
    window.location.hash = 'slide' + (slideNum - 1);
  }
}
