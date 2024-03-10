
# Heat Transfer

https://www.khanacademy.org/science/physics/thermodynamics/specific-heat-and-heat-transfer/a/what-is-thermal-conductivity

    heat transfer equation
    Q / t ​= k A ΔT​ / d
    
    Q = Heat Energy
    t = time
    k = thermal conductivity constant for material
    A = area of contact
    ΔT = difference in temperature (read delta T)
    d = distance
    
    r = thermal resistance
    we can precalculate this for each voxel material knowing it's size
    r = d / A k
    vs = voxel side length
    d = vs / 2
    A = vs ^ 2
    d / A = 1 / (2 vs)
    r = 1 / (2 vs k)
