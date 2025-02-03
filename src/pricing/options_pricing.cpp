#include "pricing/options_pricing.h"

double OptionsPricing::black_scholes(double spot, double strike, double t,
                                     double r, double sigma, bool is_call) {
  double d1 =
      (log(spot / strike) + (r + 0.5 * sigma * sigma) * t) / (sigma * sqrt(t));
  double d2 = d1 - sigma * sqrt(t);
  double Nd1 = normcdf(d1);
  double Nd2 = normcdf(d2);

  if (is_call) {
    return spot * Nd1 - strike * exp(-r * t) * Nd2;
  } else {
    return strike * exp(-r * t) * (1 - Nd2) - spot * (1 - Nd1);
  }
}

double OptionsPricing::normcdf(double x) { return 0.5 * erfc(-x * sqrt(0.5)); }
