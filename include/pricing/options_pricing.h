#ifndef OPTIONS_PRICING_H
#define OPTIONS_PRICING_H

#include <cmath>

class OptionsPricing {
public:
  static double black_scholes(double spot, double strike, double t, double r,
                              double sigma, bool is_call);

private:
  static double normcdf(double x);
};

#endif // OPTIONS_PRICING_H
