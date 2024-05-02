using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class Entity : MonoBehaviour
{
    [Header( "Data" )]
    [SerializeField] public PoolSO health;

    [SerializeField] public StatsSO focus;
    [SerializeField] public StatsSO mind;
    [SerializeField] public StatsSO body;
}